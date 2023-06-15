import { Injectable } from '@angular/core';
import { HttpClient, HttpHeaders } from '@angular/common/http';
import { Observable, of } from 'rxjs';
import { catchError, map, tap } from 'rxjs/operators';
import {NewUser, UserLoginInfo} from '../models/user';
@Injectable({
  providedIn: 'root'
})
export class UserService {
  private  urlUser = "http://127.0.0.1:8000/";
  httpOptions = {
    headers: new HttpHeaders({ 'Content-Type': 'application/json' })
  };

  constructor(private http: HttpClient) { }
  setUp(): Observable<any>{
    return this.http.get<any>(`${this.urlUser}`);
  }
  addNewUser(newUser: NewUser): Observable<boolean>{
    return this.http.post<boolean>(`${this.urlUser}sign-up`, newUser, this.httpOptions);
  }
  getUser(): Observable<UserLoginInfo>{
    return this.http.get<UserLoginInfo>(`${this.urlUser}login`);
  }
  loginUser(loginUser:UserLoginInfo):Observable<boolean>{
    return this.http.post<boolean>(`${this.urlUser}login`, loginUser, this.httpOptions);
  }
}
