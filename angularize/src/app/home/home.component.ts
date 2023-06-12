import { Component,OnInit } from '@angular/core';
import { Room } from '../models/room';

@Component({
  selector: 'app-home',
  templateUrl: './home.component.html',
  styleUrls: ['./home.component.scss']
})
export class HomeComponent implements OnInit  {
  roomList: Room[] = [];
  
  ngOnInit(): void {
    // Initialization logic goes here
    // This code will be executed when the component is initialized
  }
}
