export const idlFactory = ({ IDL }) => {
  const FlightBookingPayload = IDL.Record({
    'destination' : IDL.Text,
    'departure_time' : IDL.Nat64,
    'available_seats' : IDL.Nat32,
    'airline' : IDL.Text,
  });
  const Flight = IDL.Record({
    'id' : IDL.Nat64,
    'total_seats' : IDL.Nat32,
    'destination' : IDL.Text,
    'seats_booked' : IDL.Vec(IDL.Nat32),
    'agent_principal' : IDL.Text,
    'departure_time' : IDL.Nat64,
    'available_seats' : IDL.Nat32,
    'airline' : IDL.Text,
  });
  const Error = IDL.Variant({
    'Error' : IDL.Record({ 'msg' : IDL.Text }),
    'InvalidPayload' : IDL.Record({ 'errors' : IDL.Vec(IDL.Text) }),
    'NotFound' : IDL.Record({ 'msg' : IDL.Text }),
    'NotBooker' : IDL.Record({ 'msg' : IDL.Text }),
    'NotAgent' : IDL.Record({ 'msg' : IDL.Text }),
    'NoSeatsAvailable' : IDL.Record({ 'msg' : IDL.Text }),
  });
  const Result = IDL.Variant({ 'Ok' : Flight, 'Err' : Error });
  const BookingPayload = IDL.Record({
    'passenger_name' : IDL.Text,
    'flight_id' : IDL.Nat64,
    'seat_number' : IDL.Nat32,
  });
  const Booking = IDL.Record({
    'id' : IDL.Nat64,
    'booking_time' : IDL.Nat64,
    'booker_principal' : IDL.Text,
    'passenger_name' : IDL.Text,
    'flight_id' : IDL.Nat64,
    'seat_number' : IDL.Nat32,
  });
  const Result_1 = IDL.Variant({ 'Ok' : Booking, 'Err' : Error });
  const Result_2 = IDL.Variant({ 'Ok' : IDL.Nat32, 'Err' : Error });
  const Result_3 = IDL.Variant({ 'Ok' : IDL.Null, 'Err' : Error });
  return IDL.Service({
    'add_flight' : IDL.Func([FlightBookingPayload], [Result], []),
    'book_flight' : IDL.Func([BookingPayload], [Result_1], []),
    'check_flight_availability' : IDL.Func([IDL.Nat64], [Result_2], ['query']),
    'delete_booking' : IDL.Func([IDL.Nat64], [Result_3], []),
    'get_booking' : IDL.Func([IDL.Nat64], [Result_1], ['query']),
    'get_flight' : IDL.Func([IDL.Nat64], [Result], ['query']),
    'update_booking' : IDL.Func([IDL.Nat64, BookingPayload], [Result_1], []),
    'update_flight' : IDL.Func([IDL.Nat64, FlightBookingPayload], [Result], []),
  });
};
export const init = ({ IDL }) => { return []; };
