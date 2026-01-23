import { CommonModule } from '@angular/common';
import { Component, inject, OnInit, ChangeDetectorRef, computed } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { MatButtonModule } from '@angular/material/button';
import { MatIconModule } from '@angular/material/icon';
import { MatOptionModule } from '@angular/material/core';
import { MatSelectModule } from '@angular/material/select';
import { MatInputModule } from '@angular/material/input';
import { MatFormFieldModule } from '@angular/material/form-field';
import { Mission } from '../_models/mission';
import { MissionFilter } from '../_models/mission-filter';
import { MissionService } from '../_services/mission-service';
import { PassportService } from '../_services/passport-service';

@Component({
  selector: 'app-missions',
  standalone: true,
  imports: [
    FormsModule,
    CommonModule,
    MatButtonModule,
    MatIconModule,
    MatFormFieldModule,
    MatInputModule,
    MatSelectModule,
    MatOptionModule,
  ],
  templateUrl: './missions.html',
  styleUrl: './missions.scss',
})
export class Missions implements OnInit {
  private _missionService = inject(MissionService);
  private _passportService = inject(PassportService);
  private _cdr = inject(ChangeDetectorRef);

  isSignin = computed(() => this._passportService.isSignin());

  filter: MissionFilter = {};
  missions: Mission[] = [];
  loading = false;

  constructor() {
    this.filter = this._missionService.filter;
  }

  ngOnInit() {
    this.search();
  }

  async search() {
    this.loading = true;
    try {
      this.missions = await this._missionService.gets(this.filter);
      this._cdr.detectChanges(); // Force change detection after data update
    } catch (error) {
      console.error('Error fetching missions', error);
    } finally {
      this.loading = false;
      this._cdr.detectChanges(); // UI Update
    }
  }

  async startMission(id: number, event: Event) {
    event.stopPropagation();
    try {
      await this._missionService.inProgress(id);
      await this.search();
    } catch (error) {
      console.error('Error starting mission', error);
    }
  }

  async completeMission(id: number, event: Event) {
    event.stopPropagation();
    try {
      await this._missionService.toCompleted(id);
      await this.search();
    } catch (error) {
      console.error('Error completing mission', error);
    }
  }

  async joinMission(id: number, event: Event) {
    event.stopPropagation();
    try {
      await this._missionService.join(id);
      await this.search();
    } catch (error) {
      console.error('Error joining mission', error);
    }
  }
}
