// import { Component } from '@angular/core';

// @Component({
//   selector: 'app-passport',
//   imports: [],
//   templateUrl: './passport.html',
//   styleUrl: './passport.scss',
// })
export interface Passport {
  access_token: string;
  token_type: string;
  expires_in: number;
  display_name?: string;
  username?: string;
  avatar_url?: string;
}

export interface RegisterModel {
  display_name: string;
  username: string;
  password: string;
}
export interface LoginModel {
  username: string;
  password: string;
}
