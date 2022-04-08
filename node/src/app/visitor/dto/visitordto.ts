import { Exclude, Expose, Transform } from 'class-transformer';
import { IsOptional } from 'class-validator';

@Exclude()
export class VisitorDTO {
  @Transform(({ value }) => parseInt(value))
  @Expose()
  id: BigInt;

  @Expose()
  @IsOptional()
  modificationCounter: number;

  @Expose()
  username: string;

  @Expose()
  name: string;

  @Expose()
  password: string;

  @Expose()
  phoneNumber: string;

  @Expose()
  acceptedCommercial: boolean;

  @Expose()
  acceptedTerms: boolean;

  @Expose()
  userType: boolean;
}
