import { Component, inject } from '@angular/core';
import { BehaviorSubject } from 'rxjs';
import { CommonModule } from '@angular/common';
import { MatButtonModule } from '@angular/material/button';
import { MatIconModule } from '@angular/material/icon';
import { MatDialog, MatDialogModule } from '@angular/material/dialog';
import { MissionService } from '../_services/mission-service';
import { Mission } from '../_models/mission';
import { NewMission } from '../_dialog/new-mission/new-mission';
import { AddMission } from '../_models/add-mission';

@Component({
  selector: 'app-mission-manager',
  standalone: true,
  imports: [CommonModule, MatButtonModule, MatIconModule, MatDialogModule],
  templateUrl: './mission-manager.html',
  styleUrl: './mission-manager.scss',
})
export class MissionManager {
  private _missionService = inject(MissionService);
  private _dialog = inject(MatDialog);
  private _missionsSubject = new BehaviorSubject<Mission[]>([]);
  readonly myMissions$ = this._missionsSubject.asObservable();

  constructor() {
    this.loadMyMission();
  }

  private async loadMyMission() {
    const missions = await this._missionService.getMyMissions();
    this._missionsSubject.next(missions);
  }

  openDialog() {
    const ref = this._dialog.open(NewMission);
    ref.afterClosed().subscribe(async (addMission: AddMission) => {
      if (!addMission) return;

      const id = await this._missionService.add(addMission);
      const now = new Date();
      const newMission: Mission = {
        id,
        name: addMission.name,
        description: addMission.description,
        status: 'Open',
        chief_id: 0,
        crew_count: 0,
        created_at: now,
        updated_at: now,
      };

      const currentMissions = this._missionsSubject.value;
      this._missionsSubject.next([...currentMissions, newMission]);
    });
  }
}
