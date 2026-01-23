import { inject, Injectable } from '@angular/core';
import { MatSnackBar } from '@angular/material/snack-bar';
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
      if (
        error.status >= 400 &&
        error.status < 500 &&
        error.url &&
        (error.url.includes('login') || error.url.includes('register'))
      ) {
        const msg =
          error.error && typeof error.error === 'string' ? error.error : 'Authentication failed';
        this._snackbar.open(msg, 'ok', this.snackBarConfig);
        return throwError(() => error);
      }

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
          let msg500 =
            error.error && typeof error.error === 'string' ? error.error : 'Internal Server Error';

          // Handle specific business logic errors with snackbar
          if (
            msg500.includes('duplicate key value violates unique constraint') ||
            msg500.includes('unique_username')
          ) {
            if (msg500.includes('duplicate key value violates unique constraint')) {
              msg500 = 'Username unavailable. Please choose another username.';
            } else if (msg500.includes('unique_username')) {
              msg500 = 'Username is already taken.';
            }
            this._snackbar.open(msg500, 'ok', this.snackBarConfig);
          } else {
            // Generic 500 -> Server Error Page
            const navExtra: NavigationExtras = {
              state: {
                error: error.error,
                status: error.status,
              },
            };
            this._router.navigate(['/server-error'], navExtra);
          }
          break;
        default:
          // All other errors (including 501-511) -> Server Error Page
          const navExtraDefault: NavigationExtras = {
            state: {
              error: error.error,
              status: error.status,
            },
          };
          this._router.navigate(['/server-error'], navExtraDefault);
          break;
      }
    }
    return throwError(() => error);
  }
}
