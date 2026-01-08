import { Component, AfterViewInit } from '@angular/core';
import { animate } from 'animejs';

@Component({
  selector: 'app-not-found',
  imports: [],
  templateUrl: './not-found.html',
  styleUrl: './not-found.scss',
})
export class NotFound implements AfterViewInit {

  ngAfterViewInit() {
    animate('.row svg', {
      translateY: 10,
      loop: true,
      ease: 'inOutSine',
      direction: 'alternate'
    });

    animate('#zero', {
      translateX: 10,
      loop: true,
      ease: 'inOutSine',
      direction: 'alternate',
      scale: [{ to: 1 }, { to: 1.4 }, { to: 1, delay: 250 }],
      rotateY: { to: '+=180', delay: 200 },
    });
  }
}
