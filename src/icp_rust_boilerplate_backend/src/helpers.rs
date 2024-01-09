use ic_cdk::caller;
use ic_cdk::api::time;

use crate::types::*;

// Helper function to check whether the caller is the principal of the agent
pub fn is_caller_agent_principal(flight: &Flight) -> Result<(), Error>{
    if flight.agent_principal != caller().to_string(){
        return Err(Error::NotAgent { msg: format!("Caller is not the principal of the agent") })
    }else{
        Ok(())
    }
}
// Helper function to check whether the caller is the principal of the booker
pub fn is_caller_booker_principal(booking: &Booking) -> Result<(), Error>{
    if booking.booker_principal != caller().to_string(){
        return Err(Error::NotBooker { msg: format!("Caller is not the principal of the booker") })
    }else{
        Ok(())
    }
}
// Helper function that return a bool value on whether the trimmed string is empty
fn is_invalid_string(str: &String) -> bool{
    str.trim().is_empty()

}
// Helper function to validate the input payload when creating or updating a consultation
pub fn validate_flight_payload(payload: &FlightBookingPayload) -> Result<(), Error>{
    let mut errors: Vec<String> = Vec::new();
    if is_invalid_string(&payload.airline){
        errors.push(format!("Airline='{}' cannot be empty.", payload.airline))
    }
    if is_invalid_string(&payload.destination){
        errors.push(format!("Destination='{}' cannot be empty.", payload.destination))
    }
    if payload.available_seats == 0{
        errors.push(format!("Available seats can't be zero"));
    }
    // set to 2hrs 5mins for testing purposes
    let min_add: u64 = 7_500_000_000_000;
    let min_allowed_time = time() + min_add;
    if payload.departure_time < min_allowed_time{
        errors.push(format!("Departure time needs to be in the future and greater than the current timestamp={}", time()))
    }
    if errors.is_empty(){
        Ok(())
    }else{
        return Err(Error::InvalidPayload { errors })
    }
}
// Helper function to validate the input payload when creating or updating an advisor
pub fn validate_booking_payload(payload: &BookingPayload, flight: &Flight) -> Result<(), Error>{
    let mut errors: Vec<String> = Vec::new();
    if flight.available_seats == 0{
        errors.push(format!("There are no available seats"));
        // return the errors vec as there are no available seats to book
        return Err(Error::InvalidPayload { errors })
    }
    const TWO_HOURS_NANOSECONDS: u64 = 7_200_000_000_000;
    let last_possible_booking_time = flight.departure_time - TWO_HOURS_NANOSECONDS;
    if time() >= last_possible_booking_time{
        errors.push(
            format!("You can only book flights whose departure time is in more than two hours.",
        ));
        // return the errors vec as booking period for flight is over
        return Err(Error::InvalidPayload { errors })
    }
    if is_invalid_string(&payload.passenger_name){
        errors.push(format!("Passenger name='{}' cannot be empty.", payload.passenger_name))
    }

    if payload.seat_number >= flight.total_seats{
        errors.push(
            format!("Seat number={} cannot be greater than the total number of seats={}.", payload.seat_number, flight.total_seats
        ))
    }
    let is_seat_booked = flight.seats_booked.iter().find(|&&seat_number| seat_number == payload.seat_number);
    if is_seat_booked.is_some(){
        errors.push(
            format!("Seat number={} is already booked.", payload.seat_number
        ))
    }

    if errors.is_empty(){
        Ok(())
    }else{
        return Err(Error::InvalidPayload { errors })
    }
}