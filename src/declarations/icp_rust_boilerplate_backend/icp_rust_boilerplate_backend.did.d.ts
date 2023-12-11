import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export interface Booking {
  'id' : bigint,
  'booking_time' : bigint,
  'passenger_name' : string,
  'flight_id' : bigint,
  'seat_number' : number,
}
export type Error = { 'NotFound' : { 'msg' : string } } |
  { 'NoSeatsAvailable' : { 'msg' : string } };
export interface Flight {
  'id' : bigint,
  'destination' : string,
  'departure_time' : bigint,
  'available_seats' : number,
  'airline' : string,
}
export interface FlightBookingPayload {
  'destination' : string,
  'departure_time' : bigint,
  'passenger_name' : string,
  'available_seats' : number,
  'flight_id' : bigint,
  'airline' : string,
  'seat_number' : number,
}
export type Result = { 'Ok' : Booking } |
  { 'Err' : Error };
export type Result_1 = { 'Ok' : number } |
  { 'Err' : Error };
export type Result_2 = { 'Ok' : null } |
  { 'Err' : Error };
export type Result_3 = { 'Ok' : Flight } |
  { 'Err' : Error };
export interface _SERVICE {
  'add_flight' : ActorMethod<[FlightBookingPayload], [] | [Flight]>,
  'book_flight' : ActorMethod<[FlightBookingPayload], Result>,
  'check_flight_availability' : ActorMethod<[bigint], Result_1>,
  'delete_booking' : ActorMethod<[bigint], Result_2>,
  'delete_flight' : ActorMethod<[bigint], Result_2>,
  'get_booking' : ActorMethod<[bigint], Result>,
  'get_flight' : ActorMethod<[bigint], Result_3>,
  'update_booking' : ActorMethod<[bigint, FlightBookingPayload], Result>,
  'update_flight' : ActorMethod<[bigint, FlightBookingPayload], Result_3>,
}
