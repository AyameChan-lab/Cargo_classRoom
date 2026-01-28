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
      // Check if it's an Auth related request (Login/Register)
      // If so, show snackbar and DO NOT redirect.
      if (error.url && (error.url.includes('login') || error.url.includes('register'))) {
        let msg = 'Authentication failed';

        // Try to extraction message string
        if (error.error && typeof error.error === 'string') {
          msg = error.error;
        }

        // Handle specific scenarios
        if (error.status === 0) {
          msg = 'Unable to connect to server';
        } else if (error.status === 400 && (!error.error || typeof error.error !== 'string')) {
          if (error.url.includes('login')) {
            msg = 'Invalid username or password';
          } else {
            msg = 'Registration failed';
          }
        } else if (error.status === 500) {
          const errStr =
            error.error && typeof error.error === 'string' ? error.error : 'Internal Server Error';

          if (
            errStr.includes('duplicate key value violates unique constraint') ||
            errStr.includes('unique_username')
          ) {
            if (errStr.includes('duplicate key value violates unique constraint')) {
              msg = 'Username unavailable. Please choose another username.';
            } else if (errStr.includes('unique_username')) {
              msg = 'Username is already taken.';
            }
          } else {
            msg = errStr;
          }
        }

        this._snackbar.open(msg, 'ok', this.snackBarConfig);
        return throwError(() => error);
      }

      // Existing logic for non-Auth requests
      switch (error.status) {
        case 400:
          const msg =
            error.error && typeof error.error === 'string'
              ? error.error
              : 'invalid username or password';
          this._snackbar.open(msg, 'ok', this.snackBarConfig);
          break;
          this._router.navigate(['/not-found']);
          break;
        case 403:
          const msg403 =
            error.error && typeof error.error === 'string' ? error.error : 'Permission Denied';
          this._snackbar.open(msg403, 'ok', this.snackBarConfig);
          break;
        case 401:
          this._snackbar.open('Authorization Required', 'ok', this.snackBarConfig);
          break;
        case 409:
          this._snackbar.open('You have already joined this mission.', 'ok', this.snackBarConfig);
          break;
        case 500:
          let msg500 =
            error.error && typeof error.error === 'string' ? error.error : 'Internal Server Error';

          // Handle specific business logic errors with snackbar (redundant for auth but kept for others)
          if (
            msg500.includes('duplicate key value violates unique constraint') ||
            msg500.includes('unique_username') ||
            msg500.includes('Mission has been taken by brawler for now!')
          ) {
            // Check for mission join duplicate
            if (error.url && error.url.includes('crew/join')) {
              this._snackbar.open(
                'You have already joined this mission.',
                'ok',
                this.snackBarConfig,
              );
              break;
            }

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
