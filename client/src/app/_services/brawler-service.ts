import { HttpClient } from '@angular/common/http';
import { inject, Injectable } from '@angular/core';
import { environment } from '../../environments/environment';

@Injectable({
  providedIn: 'root',
})
export class BrawlerService {
  private _http = inject(HttpClient);

  uploadAvatar(base64String: string) {
    return this._http.post(environment.baseUrl + '/api/brawlers/avatar', {
      base64_string: base64String,
    });
  }
}
