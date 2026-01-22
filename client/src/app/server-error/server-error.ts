import { Component, inject } from '@angular/core';
import { Router, RouterLink } from '@angular/router';
import { CommonModule } from '@angular/common'; // includes JsonPipe
import { MatCardModule } from '@angular/material/card';
import { MatButtonModule } from '@angular/material/button';

@Component({
  selector: 'app-server-error',
  standalone: true,
  imports: [CommonModule, RouterLink, MatCardModule, MatButtonModule],
  templateUrl: './server-error.html',
  styleUrl: './server-error.scss',
})
export class ServerError {
  error: any;
  statusCode: number | undefined;
  private _router = inject(Router);

  constructor() {
    const nav = this._router.getCurrentNavigation();
    this.error = nav?.extras?.state?.['error'] || 'Unknown error occurred. No details available.';
    this.statusCode = nav?.extras?.state?.['status'];
  }
}
