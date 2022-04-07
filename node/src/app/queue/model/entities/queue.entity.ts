import { Entity, Column, PrimaryGeneratedColumn, OneToMany } from 'typeorm';
import { CrudValidationGroups } from '@nestjsx/crud';
import { IsDefined, IsOptional, MaxLength } from 'class-validator';
import { AccessCode } from 'src/app/accesscode/model/entities/accessCode.entity';

@Entity({ name: 'dailyqueue' })
export class Queue {
  @PrimaryGeneratedColumn()
  id: number;

  @IsDefined({ groups: [CrudValidationGroups.CREATE] })
  @IsOptional({ groups: [CrudValidationGroups.UPDATE] })
  @Column('int', { nullable: true })
  modificationcounter?: bigint;

  @IsDefined({ groups: [CrudValidationGroups.CREATE] })
  @IsOptional({ groups: [CrudValidationGroups.UPDATE] })
  @MaxLength(255)
  @Column('varchar', { length: 255, nullable: true })
  name?: string;

  @IsDefined({ groups: [CrudValidationGroups.CREATE] })
  @IsOptional({ groups: [CrudValidationGroups.UPDATE] })
  @MaxLength(255)
  @Column('varchar', { length: 255, nullable: true })
  logo?: string;

  @IsDefined({ groups: [CrudValidationGroups.CREATE] })
  @IsOptional({ groups: [CrudValidationGroups.UPDATE] })
  @MaxLength(255)
  @Column('varchar', { length: 255, nullable: true })
  currentnumber?: string;

  @IsDefined({ groups: [CrudValidationGroups.CREATE] })
  @IsOptional({ groups: [CrudValidationGroups.UPDATE] })
  @Column('timestamp', { nullable: true })
  attentiontime?: Date;

  @IsDefined({ groups: [CrudValidationGroups.CREATE] })
  @IsOptional({ groups: [CrudValidationGroups.UPDATE] })
  @Column('timestamp', { nullable: true })
  minattentiontime?: Date;

  @IsDefined({ groups: [CrudValidationGroups.CREATE] })
  @IsOptional({ groups: [CrudValidationGroups.UPDATE] })
  @Column('bool', { nullable: true })
  active?: boolean;

  @OneToMany(() => AccessCode, (accessCode) => accessCode.queue)
  accessCodes: AccessCode[];
}
