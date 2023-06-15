import { Injectable } from '@angular/core';
import { HttpClient, HttpHeaders } from '@angular/common/http';
import { Observable, of } from 'rxjs';
import { Room } from '../models/room';
import{SubjectRoom} from '../models/subject_room';
@Injectable({
  providedIn: 'root'
})
export class RoomService {
  private  urlUser = "http://127.0.0.1:8000/rooms/";
  httpOptions = {
    headers: new HttpHeaders({ 'Content-Type': 'application/json' })
  };
  
  constructor(private http: HttpClient){};
  getUserRoom(): Observable<Room[]>{
    return this.http.get<Room[]>(`${this.urlUser}`);
  }
  getUserSubRoom(id:number): Observable<SubjectRoom[]>{
    return this.http.get<SubjectRoom[]>(`${this.urlUser}{id}`);
  }
}

