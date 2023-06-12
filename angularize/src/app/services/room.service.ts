import { Injectable } from '@angular/core';
import { HttpClient, HttpHeaders } from '@angular/common/http';
import { Observable, of } from 'rxjs';
import { Room } from '../models/room';
@Injectable({
  providedIn: 'root'
})
export class RoomService {
  private  urlUser = "http://127.0.0.1:8000/";
  httpOptions = {
    headers: new HttpHeaders({ 'Content-Type': 'application/json' })
  };
  
  constructor(private http: HttpClient){};
  getUserRoom(): Observable<Room[]>{
    return this.http.get<Room[]>(`${this.urlUser}login`);
  }
}

