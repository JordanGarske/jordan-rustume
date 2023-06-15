import { Component,OnInit } from '@angular/core';
import { Room } from '../models/room';
import { RoomService } from '../services/room.service';

@Component({
  selector: 'app-home',
  templateUrl: './home.component.html',
  styleUrls: ['./home.component.scss']
})
export class HomeComponent implements OnInit  {
  roomList: Room[] = [];
  constructor(private roomService: RoomService ){
  }
  ngOnInit(): void {
    this.roomService.getUserRoom().subscribe(rooms => this.roomList = rooms)
  }
}
