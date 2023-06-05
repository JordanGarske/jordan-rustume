import { Component } from '@angular/core';
import { Router  } from '@angular/router';
import { UserLoginInfo } from 'src/app/models/user';
import { UserService } from 'src/app/services/user.service';

@Component({
  selector: 'app-login',
  templateUrl: './login.component.html',
  styleUrls: ['./login.component.scss']
})
export class LoginComponent {
  valid:string = '';
  userLogin: any;
  constructor(private userService: UserService,private router:Router){}
  onSubmit(){
    this.userService.loginUser(this.userLogin).subscribe(result =>{
      if(result){
        this.router.navigate(["/home"]);
      }
      else{
        this.valid = "failed to login"
      }
    })
  }
}
