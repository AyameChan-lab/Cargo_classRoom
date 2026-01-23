import { HttpClient } from '@angular/common/http';
import { inject, Injectable } from '@angular/core';
import { firstValueFrom } from 'rxjs';
import { environment } from '../../environments/environment';
import { Mission } from '../_models/mission';
import { MissionFilter } from '../_models/mission-filter';

@Injectable({
  providedIn: 'root',
})
export class MissionService {
  private _api_url = environment.baseUrl + '/api/v1';
  private _http = inject(HttpClient);
  filter: MissionFilter = {};

  async gets(filter: MissionFilter): Promise<Mission[]> {
    const queryString = this.toQueryString(filter);
    const url = this._api_url + '/view/gets?' + queryString;
    const observable = this._http.get<Mission[]>(url);
    const missions = await firstValueFrom(observable);
    return missions;
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
