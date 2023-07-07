import { Component, OnInit } from '@angular/core';
import { contactInfo, ContactInformation } from './test';
@Component({
  selector: 'app-resume-items',
  templateUrl: './resume-items.component.html',
  styleUrls: ['./resume-items.component.scss'],
})
export class ResumeItemsComponent {
  contactInfo?: ContactInformation[] = contactInfo;
}
