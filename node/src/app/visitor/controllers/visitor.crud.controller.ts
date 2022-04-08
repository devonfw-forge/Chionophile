import { Controller, HttpStatus, Param, Res } from '@nestjs/common';
import { Crud, CrudRequest, Override, ParsedBody, ParsedRequest } from '@nestjsx/crud';
import { CrudType } from '@devon4node/common/serializer';
import { Visitor } from '../model/entities/visitor.entity';
import { VisitorCrudService } from '../services/visitor.crud.service';
import { ApiTags } from '@nestjs/swagger';
import { Response } from 'express';
import { VisitorDTO } from '../dto/visitordto';
import { plainToClass } from 'class-transformer';

@Crud({
  model: {
    type: Visitor,
  },
})
@CrudType(Visitor)
@Controller('visitormanagement/v1/visitor')
@ApiTags('Visitor')
export class VisitorCrudController{
  constructor(public service: VisitorCrudService) {}

  @Override('createOneBase')
  async create(
     @ParsedRequest() req: CrudRequest,
     @ParsedBody() dto: Visitor,
     @Res() res: Response)
  {
    const resData = plainToClass(VisitorDTO, await this.service.createOne(req, dto));
    res.status(HttpStatus.OK).send(resData);
  }

  @Override('getOneBase')
  async get(
     @ParsedRequest() req: CrudRequest,
     @Res() res: Response)
  {
    try{
      const resData = plainToClass(VisitorDTO, await this.service.getOne(req));
      res.status(HttpStatus.OK).send(resData);
    }catch{
      res.status(HttpStatus.NOT_FOUND).send();
    }
  }

  @Override('deleteOneBase')
  async delete(
     @ParsedRequest() req: CrudRequest,
     @Param('id') id: number,
     @Res() res: Response)
  {
    try{
      await this.service.deleteOne(req);
      res.status(HttpStatus.OK).json(id);
    }catch{
      res.status(HttpStatus.NOT_FOUND).send();
    }
  }
}
