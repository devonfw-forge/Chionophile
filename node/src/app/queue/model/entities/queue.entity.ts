import { Entity, Column, PrimaryGeneratedColumn, OneToMany } from 'typeorm';
import { CrudValidationGroups } from '@nestjsx/crud';
import { IsDefined, IsOptional, MaxLength } from 'class-validator';
import { AccessCode } from 'src/app/accesscode/model/entities/accessCode.entity';
import { Transform } from 'class-transformer';

@Entity({ name: 'dailyqueue' })
export class Queue {
  @Transform(({ value }) => parseInt(value))
  @PrimaryGeneratedColumn()
  id: number;

  @Column('int', { name: 'modificationcounter' })
  modificationCounter?: number = 1;

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
  @Column('varchar', { length: 255, name: 'currentnumber' })
  currentNumber?: string;

  @Column('timestamp', { name: 'attentiontime' })
  attentionTime: Date = new Date();

  @IsDefined({ groups: [CrudValidationGroups.CREATE] })
  @IsOptional({ groups: [CrudValidationGroups.UPDATE] })
  @Column('timestamp', { name: 'minattentiontime' })
  minAttentionTime?: Date;

  @IsDefined({ groups: [CrudValidationGroups.CREATE] })
  @IsOptional({ groups: [CrudValidationGroups.UPDATE] })
  @Column('bool')
  active?: boolean;

  @OneToMany(() => AccessCode, (accessCode) => accessCode.queue)
  accessCodes: AccessCode[];
}
