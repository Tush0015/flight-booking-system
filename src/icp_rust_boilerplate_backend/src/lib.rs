#[macro_use]
extern crate serde;
use candid::{Decode, Encode};
use ic_cdk::api::time;
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{BoundedStorable, Cell, DefaultMemoryImpl, StableBTreeMap, Storable};
use std::{borrow::Cow, cell::RefCell};

type Memory = VirtualMemory<DefaultMemoryImpl>;
type IdCell = Cell<u64, Memory>;

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct Flight {
    id: u64,
    airline: String,
    destination: String,
    departure_time: u64,
    available_seats: u32,
}

impl Storable for Flight {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Flight {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

#[derive(candid::CandidType, Serialize, Deserialize, Default, Clone)]
struct Booking {
    id: u64,
    flight_id: u64,
    passenger_name: String,
    seat_number: u32,
    booking_time: u64,
}

impl Storable for Booking {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Booking {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

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

#[derive(candid::CandidType, Serialize, Deserialize, Default)]
struct FlightBookingPayload {
    flight_id: u64,
    airline: String,
    destination: String,
    departure_time: u64,
    available_seats: u32,
    passenger_name: String,
    seat_number: u32,
}

#[ic_cdk::query]
fn get_flight(flight_id: u64) -> Result<Flight, Error> {
    match _get_flight(&flight_id) {
        Some(flight) => Ok(flight),
        None => Err(Error::NotFound {
            msg: format!("a flight with id={} not found", flight_id),
        }),
    }
}

#[ic_cdk::query]
fn get_booking(booking_id: u64) -> Result<Booking, Error> {
    match _get_booking(&booking_id) {
        Some(booking) => Ok(booking),
        None => Err(Error::NotFound {
            msg: format!("a booking with id={} not found", booking_id),
        }),
    }
}

#[ic_cdk::update]
fn add_flight(flight_payload: FlightBookingPayload) -> Option<Flight> {
    let flight_id = FLIGHT_ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("cannot increment flight id counter");

    let flight = Flight {
        id: flight_id,
        airline: flight_payload.airline,
        destination: flight_payload.destination,
        departure_time: flight_payload.departure_time,
        available_seats: flight_payload.available_seats,
    };

    FLIGHTS.with(|service| service.borrow_mut().insert(flight_id, flight.clone()));
    Some(flight)
}

#[ic_cdk::update]
fn book_flight(booking_payload: FlightBookingPayload) -> Result<Booking, Error> {
    // Check if the flight exists
    let flight = match _get_flight(&booking_payload.flight_id) {
        Some(flight) => flight,
        None => {
            return Err(Error::NotFound {
                msg: format!("a flight with id={} not found", booking_payload.flight_id),
            });
        }
    };

    // Check if there are available seats
    if flight.available_seats == 0 {
        return Err(Error::NoSeatsAvailable {
            msg: "no available seats for the specified flight".to_string(),
        });
    }

    // Decrement the available seats
    let updated_flight = Flight {
        available_seats: flight.available_seats - 1,
        ..flight.clone()
    };
    FLIGHTS.with(|flights| flights.borrow_mut().insert(flight.id, updated_flight));

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
        flight_id: booking_payload.flight_id,
        passenger_name: booking_payload.passenger_name,
        seat_number: booking_payload.seat_number,
        booking_time: time(),
    };

    // Insert the booking into the BOOKINGS map
    BOOKINGS.with(|bookings| bookings.borrow_mut().insert(booking_id, booking.clone()));

    Ok(booking)
}

// Helper methods to get flight and booking by id
fn _get_flight(flight_id: &u64) -> Option<Flight> {
    FLIGHTS.with(|flights| flights.borrow().get(flight_id))
}

fn _get_booking(booking_id: &u64) -> Option<Booking> {
    BOOKINGS.with(|bookings| bookings.borrow().get(booking_id))
}

#[ic_cdk::query]
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

    // Update flight data
    flight.airline = new_data.airline;
    flight.destination = new_data.destination;
    flight.departure_time = new_data.departure_time;
    flight.available_seats = new_data.available_seats;

    // Update flight in the map
    FLIGHTS.with(|flights| flights.borrow_mut().insert(flight_id, flight.clone()));

    Ok(flight)
}

#[ic_cdk::update]
fn delete_flight(flight_id: u64) -> Result<(), Error> {
    // Check if the flight exists
    if let Some(_flight) = _get_flight(&flight_id) {
        // Remove the flight
        FLIGHTS.with(|flights| {
            flights.borrow_mut().remove(&flight_id);
        });

        Ok(())
    } else {
        Err(Error::NotFound {
            msg: format!("a flight with id={} not found", flight_id),
        })
    }
}

#[ic_cdk::update]
fn update_booking(booking_id: u64, new_data: FlightBookingPayload) -> Result<Booking, Error> {
    // Check if the booking exists
    let mut booking = match _get_booking(&booking_id) {
        Some(b) => b,
        None => {
            return Err(Error::NotFound {
                msg: format!("a booking with id={} not found", booking_id),
            });
        }
    };

    // Check if the flight exists
    let _flight = match _get_flight(&new_data.flight_id) {
        Some(f) => f,
        None => {
            return Err(Error::NotFound {
                msg: format!("a flight with id={} not found", new_data.flight_id),
            });
        }
    };

    // Update booking data
    booking.flight_id = new_data.flight_id;
    booking.passenger_name = new_data.passenger_name;
    booking.seat_number = new_data.seat_number;

    // Update booking in the map
    BOOKINGS.with(|bookings| bookings.borrow_mut().insert(booking_id, booking.clone()));

    Ok(booking)
}

#[ic_cdk::update]
fn delete_booking(booking_id: u64) -> Result<(), Error> {
    // Check if the booking exists
    if let Some(booking) = _get_booking(&booking_id) {
        // Increment the available seats for the corresponding flight
        if let Some(flight) = _get_flight(&booking.flight_id) {
            let updated_flight = Flight {
                available_seats: flight.available_seats + 1,
                ..flight.clone()
            };
            FLIGHTS.with(|flights| flights.borrow_mut().insert(flight.id, updated_flight));
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

#[ic_cdk::query]
fn check_flight_availability(flight_id: u64) -> Result<u32, Error> {
    match _get_flight(&flight_id) {
        Some(flight) => Ok(flight.available_seats),
        None => Err(Error::NotFound {
            msg: format!("a flight with id={} not found", flight_id),
        }),
    }
}

#[derive(candid::CandidType, Deserialize, Serialize)]
enum Error {
    NotFound { msg: String },
    NoSeatsAvailable { msg: String },
}

// need this to generate candid
ic_cdk::export_candid!();
