import { Component, OnInit } from '@angular/core';
import { Router  } from '@angular/router';
import { UserLoginInfo } from 'src/app/models/user';
import { UserService } from 'src/app/services/user.service';

@Component({
  selector: 'app-login',
  templateUrl: './login.component.html',
  styleUrls: ['./login.component.scss']
})
export class LoginComponent implements OnInit {
  valid:string = '';
  email?: string;
  password?: string;
  constructor(private userService: UserService,private router:Router){}
  ngOnInit(): void {
     this.userService.setUp().subscribe();
  }
  login(){
    if(this.password && this.email){
      let user: UserLoginInfo = {client_password: this.password, email: this.email}
      this.userService.loginUser(user).subscribe(result =>{
        if(result){
          this.router.navigate(["/home"]);
        }
        else{
          this.valid = "failed to login"
        }
      })
    }
    else{
      this.valid = "enter all data in each field"
    }

  }
}
