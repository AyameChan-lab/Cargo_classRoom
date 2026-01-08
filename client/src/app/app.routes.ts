import { Routes } from '@angular/router';
import { Home } from './home/home';
import { NotFound } from './not-found/not-found';
import { ServerError } from './server-error/server-error';  
import { Profile } from './profile/profile';
import { Login } from './login/login';

export const routes: Routes = [
    {path:'',component:Home},
    {path:'login',component:Login},
    {path:'profile',component:Profile},
    // {path:'server-error',component:ServerError},    
    // {path:'not-found',component:NotFound},
    {path:'**',component:NotFound}
];
