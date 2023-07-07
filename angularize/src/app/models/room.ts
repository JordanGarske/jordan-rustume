export interface Room {
    room_id: number;
    title: string;
    elucidation: string; 
    write_privilege: Boolean,
  }

export interface Message{
  room: String;
  username: String;
  message: String;
}