import { Component, inject } from '@angular/core';
import { CommonModule } from '@angular/common';
import { FormsModule } from '@angular/forms';
import { MatButtonModule } from '@angular/material/button';
import { MAT_DIALOG_DATA, MatDialogRef, MatDialogModule } from '@angular/material/dialog';
import { MatFormFieldModule } from '@angular/material/form-field';
import { MatInputModule } from '@angular/material/input';
import { MatSelectModule } from '@angular/material/select';
import { EditMission } from '../../_models/edit-mission';

@Component({
  selector: 'app-edit-mission',
  standalone: true,
  imports: [
    CommonModule,
    FormsModule,
    MatButtonModule,
    MatDialogModule,
    MatFormFieldModule,
    MatInputModule,
    MatSelectModule,
  ],
  templateUrl: './edit-mission.html',
  styleUrl: './edit-mission.scss',
})
export class EditMissionDialog {
  data: EditMission;
  statuses = ['Open', 'InProgress', 'Completed', 'Failed'];

  private readonly _dialogRef = inject(MatDialogRef<EditMissionDialog>);
  private readonly _injectedData = inject(MAT_DIALOG_DATA);

  constructor() {
    this.data = { ...this._injectedData };
  }

  onSubmit() {
    const mission = this.clean(this.data);
    this._dialogRef.close(mission);
  }

  private clean(mission: EditMission): EditMission {
    return {
      name: mission.name.trim(),
      description: mission.description?.trim() || undefined,
      status: mission.status,
    };
  }
}
