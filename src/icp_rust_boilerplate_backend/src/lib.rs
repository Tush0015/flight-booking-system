#[macro_use]
extern crate serde;
use ic_cdk::api::time;
use ic_cdk::caller;
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager};
use ic_stable_structures::{DefaultMemoryImpl, StableBTreeMap};
use std::cell::RefCell;

mod types;
use types::*;
mod helpers;
use helpers::*;

thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
        MemoryManager::init(DefaultMemoryImpl::default())
    );

    static FLIGHT_ID_COUNTER: RefCell<IdCell> = RefCell::new(
        IdCell::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))), 0)
            .expect("Cannot create a flight id counter")
    );

    static BOOKING_ID_COUNTER: RefCell<IdCell> = RefCell::new(
        IdCell::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1))), 0)
            .expect("Cannot create a booking id counter")
    );

    static FLIGHTS: RefCell<StableBTreeMap<u64, Flight, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(2)))
    ));

    static BOOKINGS: RefCell<StableBTreeMap<u64, Booking, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(3)))
    ));
}

// Function that allows users to fetch a flight stored on the canister
// Return an error if not found
#[ic_cdk::query]
fn get_flight(flight_id: u64) -> Result<Flight, Error> {
    match _get_flight(&flight_id) {
        Some(flight) => Ok(flight),
        None => Err(Error::NotFound {
            msg: format!("a flight with id={} not found", flight_id),
        }),
    }
}

// Function that allows users to fetch a booking stored on the canister
// Return an error if not found
#[ic_cdk::query]
fn get_booking(booking_id: u64) -> Result<Booking, Error> {
    match _get_booking(&booking_id) {
        Some(booking) => Ok(booking),
        None => Err(Error::NotFound {
            msg: format!("a booking with id={} not found", booking_id),
        }),
    }
}

// Function that allows users to add a flight to the canister
// Return an error if not found
#[ic_cdk::update]
fn add_flight(flight_payload: FlightBookingPayload) -> Result<Flight, Error> {
    validate_flight_payload(&flight_payload)?;
    let flight_id = FLIGHT_ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("cannot increment flight id counter");

    let flight = Flight {
        id: flight_id,
        agent_principal: caller().to_string(),
        airline: flight_payload.airline,
        destination: flight_payload.destination,
        departure_time: flight_payload.departure_time,
        available_seats: flight_payload.available_seats,
        total_seats: flight_payload.available_seats,
        seats_booked: Vec::new()
    };

    do_insert_flight(&flight.clone());
    Ok(flight)
}

// Function that allows users to add a flight stored on the canister
// Return an error if not found
#[ic_cdk::update]
fn book_flight(booking_payload: BookingPayload) -> Result<Booking, Error> {
    // Check if the flight exists
    let mut flight = match _get_flight(&booking_payload.flight_id) {
        Some(flight) => flight,
        None => {
            return Err(Error::NotFound {
                msg: format!("a flight with id={} not found", booking_payload.flight_id),
            });
        }
    };
    let _check_flight_payload = validate_booking_payload(&booking_payload, &flight)?;

    // update flight fields
    flight.available_seats = flight.available_seats - 1;
    flight.seats_booked.push(booking_payload.seat_number);

    do_insert_flight(&flight);

    // Generate a new booking id
    let booking_id = BOOKING_ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("cannot increment booking id counter");

    // Create the booking
    let booking = Booking {
        id: booking_id,
        booker_principal: caller().to_string(),
        passenger_name: booking_payload.passenger_name,
        flight_id: booking_payload.flight_id,
        seat_number: booking_payload.seat_number,
        booking_time: time(),
    };

    // Insert the booking into the BOOKINGS map
    do_insert_booking(&booking.clone());

    Ok(booking)
}

// Helper methods to get flight and booking by id
fn _get_flight(flight_id: &u64) -> Option<Flight> {
    FLIGHTS.with(|flights| flights.borrow().get(flight_id))
}

fn _get_booking(booking_id: &u64) -> Option<Booking> {
    BOOKINGS.with(|bookings| bookings.borrow().get(booking_id))
}

// Function that allows users to update a flight stored on the canister
// Return an error if not found
#[ic_cdk::update]
fn update_flight(flight_id: u64, new_data: FlightBookingPayload) -> Result<Flight, Error> {
    // Check if the flight exists
    let mut flight = match _get_flight(&flight_id) {
        Some(f) => f,
        None => {
            return Err(Error::NotFound {
                msg: format!("a flight with id={} not found", flight_id),
            });
        }
    };
    is_caller_agent_principal(&flight)?;

    // prevents a flight with bookings to be updated as this 
    // could break some of the other functionalities relying on values such as total_seats
    // and potentially cause the canister's booking state to be inaccurate
    if !flight.seats_booked.is_empty(){
        return Err(Error::Error { msg: format!("Cannot update a flight that is already booked") })
    }

    validate_flight_payload(&new_data)?;


    // Update flight data
    flight.airline = new_data.airline;
    flight.destination = new_data.destination;
    flight.departure_time = new_data.departure_time;
    flight.available_seats = new_data.available_seats;
    flight.total_seats = new_data.available_seats;

    // Update flight in the map
    do_insert_flight(&flight.clone());

    Ok(flight)
}


// Function that allows users to update a booking stored on the canister
// Return an error if booking or flight not found
#[ic_cdk::update]
fn update_booking(booking_id: u64, new_data: BookingPayload) -> Result<Booking, Error> {
    // Check if the booking exists
    let mut booking = match _get_booking(&booking_id) {
        Some(b) => b,
        None => {
            return Err(Error::NotFound {
                msg: format!("a booking with id={} not found", booking_id),
            });
        }
    };
    is_caller_booker_principal(&booking)?;

    if booking.flight_id != new_data.flight_id{
        return Err(Error::Error {
             msg: format!("Invalid flight id specified as the flight id of the booking is {}", booking.flight_id)
        })
    }

    // Check if the flight exists
    let mut flight = match _get_flight(&new_data.flight_id) {
        Some(f) => f,
        None => {
            return Err(Error::NotFound {
                msg: format!("a flight with id={} not found", new_data.flight_id),
            });
        }
    };
    let _can_update_booking = validate_booking_payload(&new_data, &flight)?;

    // Update flight fields
    
    // remove current seat number of booking from the seats_booked vec
    flight.seats_booked = flight.seats_booked.into_iter().filter(|&seat_number| seat_number != booking.seat_number).collect();
    // add new seat number to the seats_booked vec
    flight.seats_booked.push(new_data.seat_number);
    
    do_insert_flight(&flight);

    booking.passenger_name = new_data.passenger_name;
    booking.seat_number = new_data.seat_number;

    // Update booking in the map
    do_insert_booking(&booking.clone());

    Ok(booking)
}

// Function that allows users to delete a booking stored on the canister
// Return an error if not found
#[ic_cdk::update]
fn delete_booking(booking_id: u64) -> Result<(), Error> {
    // Check if the booking exists
    if let Some(booking) = _get_booking(&booking_id) {
        is_caller_booker_principal(&booking)?;
        // Increment the available seats for the corresponding flight
        if let Some(flight) = _get_flight(&booking.flight_id) {
            let updated_flight = Flight {
                available_seats: flight.available_seats.clone() + 1,
                seats_booked: flight.seats_booked.clone().into_iter().filter(|&seat_number| seat_number != booking.seat_number).collect(),
                ..flight.clone()
            };
            do_insert_flight(&updated_flight);
        }

        // Remove the booking
        BOOKINGS.with(|bookings| {
            bookings.borrow_mut().remove(&booking_id);
        });

        Ok(())
    } else {
        Err(Error::NotFound {
            msg: format!("a booking with id={} not found", booking_id),
        })
    }
}
// Function that allows users to check the availability of a flight stored on the canister
// Return an error if not found
#[ic_cdk::query]
fn check_flight_availability(flight_id: u64) -> Result<u32, Error> {
    match _get_flight(&flight_id) {
        Some(flight) => Ok(flight.available_seats),
        None => Err(Error::NotFound {
            msg: format!("a flight with id={} not found", flight_id),
        }),
    }
}

// helper method to perform insert.
fn do_insert_flight(flight: &Flight) {
    FLIGHTS.with(|flights| flights.borrow_mut().insert(flight.id, flight.clone()));
}
// helper method to perform insert.
fn do_insert_booking(booking: &Booking) {
    // Insert the booking into the BOOKINGS map
    BOOKINGS.with(|bookings| bookings.borrow_mut().insert(booking.id, booking.clone()));
}

// need this to generate candid
ic_cdk::export_candid!();
