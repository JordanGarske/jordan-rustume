import { Injectable } from '@angular/core';
import { HttpClient, HttpHeaders } from '@angular/common/http';
import { Observable, of } from 'rxjs';
import { catchError, map, tap } from 'rxjs/operators';
import { NewUser, UserLoginInfo } from '../models/user';
import { Message } from '../models/room';
@Injectable({
  providedIn: 'root',
})
export class UserService {
  private urlUser = 'http://127.0.0.1:8000/';
  httpOptions = {
    headers: new HttpHeaders({ 'Content-Type': 'application/json' }),
  };

  constructor(private http: HttpClient) {}
  setUp(): Observable<any> {
    return this.http.get<any>(`${this.urlUser}`);
  }
  addNewUser(newUser: NewUser): Observable<boolean> {
    return this.http.post<boolean>(
      `${this.urlUser}sign-up`,
      newUser,
      this.httpOptions
    );
  }
  getUser(): Observable<UserLoginInfo> {
    return this.http.get<UserLoginInfo>(`${this.urlUser}login`);
  }
  loginUser(loginUser: UserLoginInfo): Observable<boolean> {
    return this.http.post<boolean>(
      `${this.urlUser}login`,
      loginUser,
      this.httpOptions
    );
  }
  userChat(): Observable<any> {
    return new Observable((observer) => {
      const eventSource = new EventSource(`${this.urlUser}chat/events`);
      eventSource.addEventListener('message', (event: MessageEvent) =>
        observer.next(event.data)
      );
      eventSource.addEventListener('error', (event: MessageEvent) =>
        observer.error(event)
      );

      return () => {
        eventSource.close();
      };
    });
  }
  sendUserChat(message: Message): Observable<Message> {
    return this.http.post<Message>(
      `${this.urlUser}chat/message`,
      message,
      this.httpOptions
    );
  }
}
