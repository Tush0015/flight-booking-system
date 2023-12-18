import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export interface Booking {
  'id' : bigint,
  'booking_time' : bigint,
  'booker_principal' : string,
  'passenger_name' : string,
  'flight_id' : bigint,
  'seat_number' : number,
}
export interface BookingPayload {
  'passenger_name' : string,
  'flight_id' : bigint,
  'seat_number' : number,
}
export type Error = { 'Error' : { 'msg' : string } } |
  { 'InvalidPayload' : { 'errors' : Array<string> } } |
  { 'NotFound' : { 'msg' : string } } |
  { 'NotBooker' : { 'msg' : string } } |
  { 'NotAgent' : { 'msg' : string } } |
  { 'NoSeatsAvailable' : { 'msg' : string } };
export interface Flight {
  'id' : bigint,
  'total_seats' : number,
  'destination' : string,
  'seats_booked' : Uint32Array | number[],
  'agent_principal' : string,
  'departure_time' : bigint,
  'available_seats' : number,
  'airline' : string,
}
export interface FlightBookingPayload {
  'destination' : string,
  'departure_time' : bigint,
  'available_seats' : number,
  'airline' : string,
}
export type Result = { 'Ok' : Flight } |
  { 'Err' : Error };
export type Result_1 = { 'Ok' : Booking } |
  { 'Err' : Error };
export type Result_2 = { 'Ok' : number } |
  { 'Err' : Error };
export type Result_3 = { 'Ok' : null } |
  { 'Err' : Error };
export interface _SERVICE {
  'add_flight' : ActorMethod<[FlightBookingPayload], Result>,
  'book_flight' : ActorMethod<[BookingPayload], Result_1>,
  'check_flight_availability' : ActorMethod<[bigint], Result_2>,
  'delete_booking' : ActorMethod<[bigint], Result_3>,
  'get_booking' : ActorMethod<[bigint], Result_1>,
  'get_flight' : ActorMethod<[bigint], Result>,
  'update_booking' : ActorMethod<[bigint, BookingPayload], Result_1>,
  'update_flight' : ActorMethod<[bigint, FlightBookingPayload], Result>,
}
