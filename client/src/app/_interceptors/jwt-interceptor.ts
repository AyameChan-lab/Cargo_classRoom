import { HttpInterceptorFn } from '@angular/common/http';
import { inject } from '@angular/core';
import { PassportService } from '../_services/passport-service';

export const jwtInterceptor: HttpInterceptorFn = (req, next) => {
  const passportService = inject(PassportService);
  const passport = passportService.data();

  if (passport?.access_token) {
    req = req.clone({
      setHeaders: {
        Authorization: `Bearer ${passport.access_token}`,
      },
    });
  }

  return next(req);
};
