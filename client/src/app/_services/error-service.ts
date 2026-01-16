import { inject, Injectable } from '@angular/core';
import { MatSnackBar, MatSnackBarRef } from '@angular/material/snack-bar';
import { NavigationExtras, Router } from '@angular/router';
import { Observable, throwError } from 'rxjs';
import { MatSnackBarConfig } from '@angular/material/snack-bar';

@Injectable({
  providedIn: 'root',
})
export class ErrorService {
  private _router = inject(Router);
  private _snackbar = inject(MatSnackBar);
  private snackBarConfig: MatSnackBarConfig = {
    duration: 5000,
    verticalPosition: 'top',
    horizontalPosition: 'right',
  };

  handleError(error: any): Observable<never> {
    if (error) {
      switch (error.status) {
        case 400:
          const msg =
            error.error && typeof error.error === 'string'
              ? error.error
              : 'invalid username or password';
          this._snackbar.open(msg, 'ok', this.snackBarConfig);
          break;
        case 404:
          this._router.navigate(['/not-found']);
          break;
        case 401:
          this._snackbar.open('Authorization Required', 'ok', this.snackBarConfig);
          break;
        case 500:
          this._router.navigate(['/server-error']);
          break;
        case 501:
          this._router.navigate(['/server-error']);
          break;
        case 502:
          this._router.navigate(['/server-error']);
          break;
        case 503:
          this._router.navigate(['/server-error']);
          break;
        case 504:
          this._router.navigate(['/server-error']);
          break;
        case 505:
          this._router.navigate(['/server-error']);
          break;
        case 506:
          this._router.navigate(['/server-error']);
          break;
        case 507:
          this._router.navigate(['/server-error']);
          break;
        case 508:
          this._router.navigate(['/server-error']);
          break;
        case 509:
          this._router.navigate(['/server-error']);
          break;
        case 510:
          this._router.navigate(['/server-error']);
          break;
        case 511:
          const navExtra: NavigationExtras = {
            state: {
              error: error.error,
            },
          };
          this._router.navigate(['/server-error'], navExtra);
          break;
        default:
          this._snackbar.open(
            'some thing went wrong >_<))) pls try againg later ><',
            'ok',
            this.snackBarConfig
          );
          break;
      }
    }
    return throwError(() => error);
  }
}
