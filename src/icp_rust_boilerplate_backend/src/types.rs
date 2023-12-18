use candid::{Decode, Encode};
use ic_stable_structures::memory_manager::VirtualMemory;
use ic_stable_structures::{BoundedStorable, Cell, DefaultMemoryImpl, Storable};
use std::borrow::Cow;

pub type Memory = VirtualMemory<DefaultMemoryImpl>;
pub type IdCell = Cell<u64, Memory>;

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
pub struct Flight {
    pub id: u64,
    pub airline: String,
    pub agent_principal: String,
    pub destination: String,
    pub departure_time: u64,
    pub available_seats: u32,
    pub total_seats: u32,
    pub seats_booked: Vec<u32>
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
pub struct Booking {
    pub id: u64,
    pub flight_id: u64,
    pub booker_principal: String,
    pub passenger_name: String,
    pub seat_number: u32,
    pub booking_time: u64,
}
#[derive(candid::CandidType, Serialize, Deserialize, Default, Clone)]
pub struct BookingPayload {
    pub flight_id: u64,
    pub passenger_name: String,
    pub seat_number: u32,
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

#[derive(candid::CandidType, Serialize, Deserialize, Default)]
pub struct FlightBookingPayload {
    pub airline: String,
    pub destination: String,
    pub departure_time: u64,
    pub available_seats: u32,
}

#[derive(candid::CandidType, Deserialize, Serialize)]
pub enum Error {
    NotFound { msg: String },
    NoSeatsAvailable { msg: String },
    InvalidPayload{errors: Vec<String>},
    NotAgent{msg: String},
    NotBooker{msg: String},
    Error{msg: String}
}