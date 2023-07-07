import { Component, OnInit } from '@angular/core';
import { UserService } from '../services/user.service';
import { Message } from '../models/room';
import { Observable } from 'rxjs';
@Component({
  selector: 'app-chatroom',
  templateUrl: './chatroom.component.html',
  styleUrls: ['./chatroom.component.scss'],
})
export class ChatroomComponent implements OnInit {
  chatlog: String[] = [];
  inputValue: String = '';
  constructor(private userService: UserService) {}
  ngOnInit(): void {
    this.userService.userChat().subscribe((Message) => {
      this.chatlog.push(Message);
      console.log('here');
    });
  }

  submit() {
    let x: Message = {
      room: '101',
      username: 'Jordan',
      message: this.inputValue,
    };
    this.userService
      .sendUserChat(x)
      .subscribe((message) => console.log(message));
  }
}
