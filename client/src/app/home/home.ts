import { Component, inject } from '@angular/core';
import { PassportService } from '../_services/passport-service';
import { Router } from '@angular/router';

@Component({
  selector: 'app-home',
  imports: [],
  templateUrl: './home.html',
  styleUrl: './home.scss',
})
export class Home {
  private _passport = inject(PassportService)
  private _router = inject(Router)

  constructor() {
    if (!this._passport.data()) {
      this._router.navigate(['/login'])
    }
  }
}
