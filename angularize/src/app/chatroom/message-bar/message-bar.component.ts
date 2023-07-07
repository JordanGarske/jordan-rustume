import { Component } from '@angular/core';
import { Message } from 'src/app/models/room';
import { UserService } from 'src/app/services/user.service';

@Component({
  selector: 'app-message-bar',
  templateUrl: './message-bar.component.html',
  styleUrls: ['./message-bar.component.scss'],
})
export class MessageBarComponent {
  chatlog: String[] = [];
  inputValue: String = '';
  constructor(private userService: UserService) {}

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
