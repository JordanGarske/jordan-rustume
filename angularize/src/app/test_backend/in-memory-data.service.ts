import { Injectable } from '@angular/core';
import { InMemoryDbService } from 'angular-in-memory-web-api';
import {User} from '../models/user' 
@Injectable({
  providedIn: 'root'
})
export class InMemoryDataService implements InMemoryDbService {

  createDb() {
    const users = [
      {client_id: 1,first_name: "Jordan",last_name: "Garske",client_password: "f",email: "f",admin_privilege: true,}
    ];
    return {heroes: users};
  }
}


