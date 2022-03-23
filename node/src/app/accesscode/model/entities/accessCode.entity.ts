import { Entity, Column, PrimaryGeneratedColumn, ManyToOne, OneToOne, JoinColumn } from 'typeorm';
import { CrudValidationGroups } from '@nestjsx/crud';
import { IsDefined, IsOptional, MaxLength } from 'class-validator';
import { Visitor } from '../../../visitor/model/entities/visitor.entity';
import { Queue } from '../../../queue/model/entities/queue.entity';

@Entity({ name: 'accesscode' })
export class AccessCode {
  @PrimaryGeneratedColumn()
  id: number;

  @IsDefined({ groups: [CrudValidationGroups.CREATE] })
  @IsOptional({ groups: [CrudValidationGroups.UPDATE] })
  @Column('int', { name: 'modificationcounter', nullable: true })
  modificationCounter?: number;

  @IsDefined({ groups: [CrudValidationGroups.CREATE] })
  @IsOptional({ groups: [CrudValidationGroups.UPDATE] })
  @MaxLength(255)
  @Column('varchar', { name: 'ticketnumber', length: 255, nullable: true })
  ticketNumber?: string;

  @IsDefined({ groups: [CrudValidationGroups.CREATE] })
  @IsOptional({ groups: [CrudValidationGroups.UPDATE] })
  @Column('timestamp', {name: 'creationtime',  nullable: true })
  creationTime?: Date;

  @IsDefined({ groups: [CrudValidationGroups.CREATE] })
  @IsOptional({ groups: [CrudValidationGroups.UPDATE] })
  @Column('timestamp', {name: 'starttime', nullable: true })
  startTime?: Date;

  @IsDefined({ groups: [CrudValidationGroups.CREATE] })
  @IsOptional({ groups: [CrudValidationGroups.UPDATE] })
  @Column('timestamp', {name: 'endtime', nullable: true })
  endTime?: Date;

  @IsDefined({ groups: [CrudValidationGroups.CREATE] })
  @IsOptional({ groups: [CrudValidationGroups.UPDATE] })
  @Column('int', {name: 'idvisitor', nullable: true })
  @OneToOne(() => Visitor)
  @JoinColumn({ name: 'id' })
  visitorId?: number;

  @IsDefined({ groups: [CrudValidationGroups.CREATE] })
  @IsOptional({ groups: [CrudValidationGroups.UPDATE] })
  @Column('int', {name: 'idqueue', nullable: true })
  @ManyToOne(() => Queue)
  @JoinColumn({ name: 'id' })
  queueId?: number;
}
