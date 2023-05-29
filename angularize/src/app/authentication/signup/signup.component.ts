import { Component } from '@angular/core';
import { User, NewUser } from 'src/app/models/user';
import { UserService } from 'src/app/services/user.service';
@Component({
  selector: 'app-signup',
  templateUrl: './signup.component.html',
  styleUrls: ['./signup.component.scss']
})
export class SignupComponent {
  userData: NewUser= {
    client_id: 1,
    first_name: '',
    last_name: '',
    client_password: '',
    email: '',
  };
  createdUser?:User;
  users:User = {
    client_id: 1,
    first_name: '',
    last_name: '',
    client_password: '',
    email: '',
    admin_privilege: true
  };
 constructor(private userService: UserService){
 }
 onSubmit() {
  this.userService.getUser().subscribe(users => this.users = users  );
  //  this.userService.addNewUser(this.userData).subscribe(user => this.createdUser = user );
 }
 login() {
  this.userService.getUser().subscribe(users => users  )
 }
}
