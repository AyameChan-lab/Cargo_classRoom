import { Component, computed, inject, signal, Signal } from '@angular/core';
import { MatToolbarModule, } from '@angular/material/toolbar';
import { MatButtonModule } from '@angular/material/button';
import { Router, RouterLink } from '@angular/router';
import { PassportService } from '../_services/passport-service';
import { MatMenuModule } from '@angular/material/menu';

@Component({
  selector: 'app-navbar',
  imports: [MatToolbarModule, MatButtonModule, RouterLink, MatMenuModule],
  templateUrl: './navbar.html',
  styleUrl: './navbar.scss',
})
export class Navbar {
  private _passport = inject(PassportService);
  private _router = inject(Router);
  
  display_name:Signal<string|undefined>
  avatar_url:Signal<string|undefined>

  constructor(){
    this.display_name = computed(() => this._passport.data()?.display_name)
    this.avatar_url = computed(() => this._passport.data()?.avatar_url || '/assets/default.avatar.jpg')
  }

  logout() {
    this._passport.logout();
    this._router.navigate(['/login']);
  }
}
