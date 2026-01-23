import { Routes } from '@angular/router';
import { Home } from './home/home';
import { NotFound } from './not-found/not-found';
import { ServerError } from './server-error/server-error';
import { Profile } from './profile/profile';
import { Login } from './login/login';
import { authGuard } from './_guard/auth-guard';
import { Missions } from './missions/missions';
import { MissionManager } from './mission-manager/mission-manager';

export const routes: Routes = [
  { path: '', component: Home },
  { path: 'login', component: Login },
  { path: 'register', component: Login },
  {
    path: 'profile',
    component: Profile,
    canActivate: [authGuard],
    runGuardsAndResolvers: 'always',
  },
  {
    path: 'mission-manager',
    component: MissionManager,
    runGuardsAndResolvers: 'always',
    canActivate: [authGuard],
  },
  {
    path: 'chief',
    redirectTo: 'mission-manager',
    pathMatch: 'full',
  },
  { path: 'missions', component: Missions },
  { path: 'server-error', component: ServerError },
  { path: 'not-found', component: NotFound },
  { path: '**', component: NotFound },
];
