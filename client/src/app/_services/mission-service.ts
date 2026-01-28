import { HttpClient } from '@angular/common/http';
import { inject, Injectable } from '@angular/core';
import { firstValueFrom } from 'rxjs';
import { environment } from '../../environments/environment';
import { Mission } from '../_models/mission';
import { MissionFilter } from '../_models/mission-filter';
import { AddMission } from '../_models/add-mission';
import { EditMission } from '../_models/edit-mission';

@Injectable({
  providedIn: 'root',
})
export class MissionService {
  private _api_url = environment.baseUrl + '/api';
  private _http = inject(HttpClient);
  filter: MissionFilter = {};

  async add(mission: AddMission): Promise<number> {
    const url = this._api_url + '/missions-management';
    const observable = this._http.post<{ mission_id: number }>(url, mission);
    const resp = await firstValueFrom(observable);
    return resp.mission_id;
  }

  async edit(id: number, mission: EditMission): Promise<void> {
    const url = `${this._api_url}/missions-management/${id}`;
    await firstValueFrom(this._http.patch(url, mission));
  }

  async delete(id: number): Promise<void> {
    const url = `${this._api_url}/missions-management/${id}`;
    await firstValueFrom(this._http.delete(url));
  }

  async getMyMissions(): Promise<Mission[]> {
    const url = this._api_url + '/brawlers/missions';
    const observable = this._http.get<Mission[]>(url);
    const missions = await firstValueFrom(observable);
    return missions;
  }

  async getJoinedMissions(): Promise<Mission[]> {
    const url = this._api_url + '/brawlers/missions/joined';
    const observable = this._http.get<Mission[]>(url);
    const missions = await firstValueFrom(observable);
    return missions;
  }

  async gets(filter: MissionFilter): Promise<Mission[]> {
    const queryString = this.toQueryString(filter);
    const url = this._api_url + '/view/gets?' + queryString;
    const observable = this._http.get<Mission[]>(url);
    const missions = await firstValueFrom(observable);
    return missions;
  }

  async inProgress(id: number): Promise<void> {
    const url = `${this._api_url}/mission/in-progress/${id}`;
    await firstValueFrom(this._http.patch(url, {}, { responseType: 'text' }));
  }

  async toCompleted(id: number): Promise<void> {
    const url = `${this._api_url}/mission/to-completed/${id}`;
    await firstValueFrom(this._http.patch(url, {}, { responseType: 'text' }));
  }

  async toFailed(id: number): Promise<void> {
    const url = `${this._api_url}/mission/to-failed/${id}`;
    await firstValueFrom(this._http.patch(url, {}, { responseType: 'text' }));
  }

  async join(id: number): Promise<void> {
    const url = `${this._api_url}/crew/join/${id}`;
    await firstValueFrom(this._http.post(url, {}, { responseType: 'text' }));
  }

  async leave(id: number): Promise<void> {
    const url = `${this._api_url}/crew/leave/${id}`;
    await firstValueFrom(this._http.delete(url, { responseType: 'text' }));
  }

  private toQueryString(filter: MissionFilter): string {
    this.filter = filter;
    const params: string[] = [];
    if (filter.name && filter.name.trim()) {
      params.push(`name=${encodeURIComponent(filter.name.trim())}`);
    }
    if (filter.status) {
      params.push(`status=${encodeURIComponent(filter.status)}`);
    }
    return params.join('&');
  }
}
