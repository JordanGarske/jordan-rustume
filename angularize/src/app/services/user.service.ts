import { Injectable } from '@angular/core';
import { HttpClient, HttpHeaders } from '@angular/common/http';
import { Observable, of } from 'rxjs';
import { catchError, map, tap } from 'rxjs/operators';
import {NewUser, User} from '../models/user';
@Injectable({
  providedIn: 'root'
})
export class UserService {
  private  urlUser = "http://127.0.0.1:8000/";
  httpOptions = {
    headers: new HttpHeaders({ 'Content-Type': 'application/json' })
  };

  constructor(private http: HttpClient) { }
  addNewUser(newUser: NewUser): Observable<boolean>{
    return this.http.post<boolean>(`${this.urlUser}sign-up`, newUser, this.httpOptions);
  }
  getUser(): Observable<User>{
    return this.http.get<User>(`${this.urlUser}login`);
  }
  loginUser(loginUser:User):Observable<boolean>{
    return this.http.post<boolean>(`${this.urlUser}login`, loginUser, this.httpOptions);
  }
}
