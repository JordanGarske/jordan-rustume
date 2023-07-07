import { Component } from '@angular/core';
import { Location } from '@angular/common';
import { User, NewUser } from 'src/app/models/user';
import { UserService } from 'src/app/services/user.service';
@Component({
  selector: 'app-signup',
  templateUrl: './signup.component.html',
  styleUrls: ['./signup.component.scss'],
})
export class SignupComponent {
  create: string = '';
  userData: NewUser = {
    client_id: 1,
    first_name: '',
    last_name: '',
    client_password: '',
    email: '',
  };
  createdUser?: User;
  users: User = {
    client_id: 1,
    first_name: '',
    last_name: '',
    client_password: '',
    email: '',
    admin_privilege: true,
  };
  constructor(private userService: UserService, private location: Location) {}
  onSubmit() {
    this.userService.addNewUser(this.userData).subscribe((isUserCreated) => {
      if (isUserCreated) {
        this.location.back();
      } else {
        this.create = 'please try different info ';
      }
    });
    //  this.userService.addNewUser(this.userData).subscribe(user => this.createdUser = user );
  }
  login() {
    this.userService.addNewUser;
  }
}
