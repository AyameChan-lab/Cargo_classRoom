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
import { PassportService } from '../_services/passport-service';
import { EditMissionDialog } from '../_dialog/edit-mission/edit-mission';
import { EditMission } from '../_models/edit-mission';

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
  private _passportService = inject(PassportService);
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
        chief_display_name: this._passportService.data()?.display_name || 'Me',
        crew_count: 0,
        created_at: now,
        updated_at: now,
      };

      const currentMissions = this._missionsSubject.value;
      this._missionsSubject.next([...currentMissions, newMission]);
    });
  }

  openEditDialog(mission: Mission) {
    const ref = this._dialog.open(EditMissionDialog, {
      data: {
        name: mission.name,
        description: mission.description,
        status: mission.status,
      } as EditMission,
    });

    ref.afterClosed().subscribe(async (editedMission: EditMission) => {
      if (!editedMission) return;

      await this._missionService.edit(mission.id, editedMission);

      // Update local state directly with new data + new timestamp
      const currentMissions = this._missionsSubject.value;
      const index = currentMissions.findIndex((m) => m.id === mission.id);
      if (index !== -1) {
        const updatedMission = {
          ...currentMissions[index],
          ...editedMission,
          updated_at: new Date(), // User wants immediate update
        };
        const updatedList = [...currentMissions];
        updatedList[index] = updatedMission;
        this._missionsSubject.next(updatedList);
      }
    });
  }
}
