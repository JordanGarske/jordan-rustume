import { Component, Input } from '@angular/core';
import { Room } from 'src/app/models/room';

@Component({
  selector: 'app-room-list',
  templateUrl: './room-list.component.html',
  styleUrls: ['./room-list.component.scss']
})
export class RoomListComponent {
  @Input() rooms: Room[] = [];

  onItemClick(room: Room) {
    // Handle the click event for each room item
    console.log('Clicked Room:', room);
  }
}
