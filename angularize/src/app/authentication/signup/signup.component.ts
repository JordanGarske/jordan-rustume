import { Component } from '@angular/core';
import { User } from 'src/app/models/user';
import { UserService } from 'src/app/services/user.service';
@Component({
  selector: 'app-signup',
  templateUrl: './signup.component.html',
  styleUrls: ['./signup.component.scss']
})
export class SignupComponent {
  userData: User= {
    client_id: 1,
    first_name: '',
    last_name: '',
    client_password: '',
    email: '',
    admin_privilege: false
  };
  createdUser?:User;
 constructor(private userService: UserService){
 }
 onSubmit() {
   this.userService.addNewUser(this.userData).subscribe(user => this.createdUser = user );
 }
}
