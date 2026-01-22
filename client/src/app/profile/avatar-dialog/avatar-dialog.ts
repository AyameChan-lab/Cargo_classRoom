import { Component, inject, ChangeDetectorRef } from '@angular/core';
import { MatDialogRef, MatDialogModule } from '@angular/material/dialog';
import { MatButtonModule } from '@angular/material/button';
import { MatIconModule } from '@angular/material/icon';
import { CommonModule } from '@angular/common';
import { BrawlerService } from '../../_services/brawler-service';
import { MatSnackBar } from '@angular/material/snack-bar';

@Component({
  selector: 'app-avatar-dialog',
  standalone: true,
  imports: [CommonModule, MatDialogModule, MatButtonModule, MatIconModule],
  templateUrl: './avatar-dialog.html',
  styleUrl: './avatar-dialog.scss',
})
export class AvatarDialog {
  private _dialogRef = inject(MatDialogRef<AvatarDialog>);
  private _brawlerService = inject(BrawlerService);
  private _snackBar = inject(MatSnackBar);
  private _cdr = inject(ChangeDetectorRef);

  previewUrl: string | null = null;
  selectedFile: File | null = null;
  base64String: string | null = null;
  isUploading = false;

  onFileSelected(event: any) {
    const file = event.target.files[0];
    if (file) {
      if (!file.type.startsWith('image/')) {
        this._snackBar.open('Please select an image file', 'Close', { duration: 3000 });
        return;
      }
      if (file.size > 2 * 1024 * 1024) {
        // 2MB
        this._snackBar.open('File size should not exceed 2MB', 'Close', { duration: 3000 });
        return;
      }

      this.selectedFile = file;
      const reader = new FileReader();
      reader.onload = (e: any) => {
        this.previewUrl = e.target.result;
        this.base64String = e.target.result;
        this._cdr.detectChanges();
      };
      reader.readAsDataURL(file);
    }
  }

  upload() {
    if (!this.base64String) return;

    this.isUploading = true;
    this._brawlerService.uploadAvatar(this.base64String).subscribe({
      next: (res) => {
        this.isUploading = false;
        this._dialogRef.close(res);
        this._snackBar.open('Avatar updated successfully!', 'Close', { duration: 3000 });
      },
      error: (err) => {
        this.isUploading = false;
        // Error service will handle global errors, but we stop loading here
      },
    });
  }

  close() {
    this._dialogRef.close();
  }
}
