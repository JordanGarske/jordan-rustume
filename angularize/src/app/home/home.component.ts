import { Component,OnInit } from '@angular/core';
import { Room } from '../models/room';
import { RoomService } from '../services/room.service';

@Component({
  selector: 'app-home',
  templateUrl: './home.component.html',
  styleUrls: ['./home.component.scss']
})
export class HomeComponent implements OnInit  {
  roomList: Room[] = [
    {
      room_id: 1,
      title: 'Room 1',
      elucidation: 'This is the first room.',
    },
    {
      room_id: 2,
      title: 'Room 2',
      elucidation: 'This is the second room.',
    },
    {
      room_id: 3,
      title: 'Room 3',
      elucidation: 'This is the third room.',
    },
  ];
  constructor(private roomService: RoomService ){
  }
  ngOnInit(): void {

    // this.roomService.getUserRoom().subscribe(rooms => this.roomList = rooms)
  }
}
