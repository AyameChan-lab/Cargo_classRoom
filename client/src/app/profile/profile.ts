import { Component, inject } from '@angular/core';
import { Router } from '@angular/router';
import { MatDialog, MatDialogModule } from '@angular/material/dialog';
import { PassportService } from '../_services/passport-service';
import { CommonModule } from '@angular/common';
import { MatButtonModule } from '@angular/material/button';
import { MatIconModule } from '@angular/material/icon';
import { MatCardModule } from '@angular/material/card';
import { AvatarDialog } from './avatar-dialog/avatar-dialog';

@Component({
  selector: 'app-profile',
  standalone: true,
  imports: [CommonModule, MatDialogModule, MatButtonModule, MatIconModule, MatCardModule],
  templateUrl: './profile.html',
  styleUrl: './profile.scss',
})
export class Profile {
  passportService = inject(PassportService);
  private _dialog = inject(MatDialog);
  private _router = inject(Router);

  openAvatarDialog() {
    const dialogRef = this._dialog.open(AvatarDialog, {
      width: '400px',
    });

    dialogRef.afterClosed().subscribe((result) => {
      if (result) {
        // Result is the uploaded image object { url, public_id }
        this.passportService.updateAvatar(result.url);
      }
    });
  }

  logout() {
    this.passportService.logout();
    this._router.navigate(['/login']);
  }
}
