import { Exclude, Expose, Transform } from "class-transformer";

@Exclude()
export class AccessCodeDTO {
  @Expose()
  @Transform(({ value }) => parseInt(value))
  visitorId: number;
  @Expose()
  @Transform(({ value }) => parseInt(value))
  queueId: number;
}
