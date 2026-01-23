import { Component, inject, ChangeDetectorRef } from '@angular/core';
import { Router } from '@angular/router';
import { MatDialog, MatDialogModule } from '@angular/material/dialog';
import { PassportService } from '../_services/passport-service';
import { CommonModule } from '@angular/common';
import { MatButtonModule } from '@angular/material/button';
import { MatIconModule } from '@angular/material/icon';
import { MatCardModule } from '@angular/material/card';
import { AvatarDialog } from './avatar-dialog/avatar-dialog';
import { MissionService } from '../_services/mission-service';
import { Mission } from '../_models/mission';

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
  private _missionService = inject(MissionService);

  private _cdr = inject(ChangeDetectorRef);

  joinedMissions: Mission[] = [];

  constructor() {
    this.loadJoinedMissions();
  }

  async loadJoinedMissions() {
    try {
      this.joinedMissions = await this._missionService.getJoinedMissions();
      console.log('Joined Missions:', this.joinedMissions);
      this._cdr.detectChanges(); // Fix NG0100
    } catch (error) {
      console.error('Failed to load joined missions', error);
    }
  }

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
