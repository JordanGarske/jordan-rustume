import { ComponentFixture, TestBed } from '@angular/core/testing';

import { ResumeItemsComponent } from './resume-items.component';

describe('ResumeItemsComponent', () => {
  let component: ResumeItemsComponent;
  let fixture: ComponentFixture<ResumeItemsComponent>;

  beforeEach(() => {
    TestBed.configureTestingModule({
      declarations: [ResumeItemsComponent]
    });
    fixture = TestBed.createComponent(ResumeItemsComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
