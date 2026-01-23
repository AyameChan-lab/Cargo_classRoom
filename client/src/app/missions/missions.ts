import { Component, inject } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { MissionFilter } from '../_models/mission-filter';
import { Mission } from '../_models/mission';
import { MissionService } from '../_services/mission-service';
import { JsonPipe } from '@angular/common';

@Component({
  selector: 'app-missions',
  imports: [FormsModule, JsonPipe],
  templateUrl: './missions.html',
  styleUrl: './missions.scss',
})
export class Missions {
  private _missionService = inject(MissionService);
  filter: MissionFilter = {};
  missions: Mission[] = [];

  constructor() {
    this.filter = this._missionService.filter;
  }

  async onSubmit() {
    this.missions = await this._missionService.gets(this.filter);
  }
}
