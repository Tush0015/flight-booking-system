export const idlFactory = ({ IDL }) => {
  const FlightBookingPayload = IDL.Record({
    'destination' : IDL.Text,
    'departure_time' : IDL.Nat64,
    'passenger_name' : IDL.Text,
    'available_seats' : IDL.Nat32,
    'flight_id' : IDL.Nat64,
    'airline' : IDL.Text,
    'seat_number' : IDL.Nat32,
  });
  const Flight = IDL.Record({
    'id' : IDL.Nat64,
    'destination' : IDL.Text,
    'departure_time' : IDL.Nat64,
    'available_seats' : IDL.Nat32,
    'airline' : IDL.Text,
  });
  const Booking = IDL.Record({
    'id' : IDL.Nat64,
    'booking_time' : IDL.Nat64,
    'passenger_name' : IDL.Text,
    'flight_id' : IDL.Nat64,
    'seat_number' : IDL.Nat32,
  });
  const Error = IDL.Variant({
    'NotFound' : IDL.Record({ 'msg' : IDL.Text }),
    'NoSeatsAvailable' : IDL.Record({ 'msg' : IDL.Text }),
  });
  const Result = IDL.Variant({ 'Ok' : Booking, 'Err' : Error });
  const Result_1 = IDL.Variant({ 'Ok' : IDL.Nat32, 'Err' : Error });
  const Result_2 = IDL.Variant({ 'Ok' : IDL.Null, 'Err' : Error });
  const Result_3 = IDL.Variant({ 'Ok' : Flight, 'Err' : Error });
  return IDL.Service({
    'add_flight' : IDL.Func([FlightBookingPayload], [IDL.Opt(Flight)], []),
    'book_flight' : IDL.Func([FlightBookingPayload], [Result], []),
    'check_flight_availability' : IDL.Func([IDL.Nat64], [Result_1], ['query']),
    'delete_booking' : IDL.Func([IDL.Nat64], [Result_2], []),
    'delete_flight' : IDL.Func([IDL.Nat64], [Result_2], []),
    'get_booking' : IDL.Func([IDL.Nat64], [Result], ['query']),
    'get_flight' : IDL.Func([IDL.Nat64], [Result_3], ['query']),
    'update_booking' : IDL.Func(
        [IDL.Nat64, FlightBookingPayload],
        [Result],
        [],
      ),
    'update_flight' : IDL.Func(
        [IDL.Nat64, FlightBookingPayload],
        [Result_3],
        ['query'],
      ),
  });
};
export const init = ({ IDL }) => { return []; };
