import { Injectable } from '@nestjs/common';
import { InjectRepository } from '@nestjs/typeorm';
import { TypeOrmCrudService } from '@nestjsx/crud-typeorm';
import { Repository } from 'typeorm';
import { AccessCodeDTO } from '../dto/accessCode';
import { AccessCodeResponse } from '../dto/accessCodeResponse';
import { AccessCode } from '../model/entities/accessCode.entity';
import { plainToClass } from 'class-transformer';

@Injectable()
export class AccessCodeCrudService extends TypeOrmCrudService<AccessCode> {
  constructor(
    @InjectRepository(AccessCode) private repoCode: Repository<AccessCode>,
  ) {
    super(repoCode);
  }

  async createOneMod(dto: AccessCodeDTO): Promise<AccessCodeResponse | undefined> {
    const existVisitor = await this.repoCode.findOne({ where: { queueId: dto.queueId, visitorId: dto.visitorId } });

    if (existVisitor != undefined) {
      return plainToClass(AccessCodeResponse, existVisitor);
    }

    const accessCode: AccessCode = new AccessCode();
    accessCode.creationTime = new Date();
    accessCode.startTime = new Date();
    accessCode.queueId = dto.queueId;
    accessCode.visitorId = dto.visitorId;
    accessCode.modificationCounter = 0;
    const insertAccessCode = plainToClass(AccessCodeResponse, await this.repoCode.save(accessCode));
    return insertAccessCode;
  }

  async deleteOneMod(id: number): Promise<void> {
    await this.repoCode.delete(id);
  }
}
