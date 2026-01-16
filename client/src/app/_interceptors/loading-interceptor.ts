import { HttpInterceptorFn } from '@angular/common/http';
import { inject } from '@angular/core';
import { Loading } from '../_services/loading';
import { delay, finalize } from 'rxjs';

export const loadingInterceptor: HttpInterceptorFn = (req, next) => {
  const spinner = inject(Loading);
  spinner.loading();
  return next(req).pipe(
    delay(2000),
    finalize(()=>{
      spinner.idle();
    })
  )
};
